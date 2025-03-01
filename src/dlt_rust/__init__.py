from dlt_rust._core import get_player_profiles, PyClient, get_player_games
import dlt
from typing import List, Sequence
from dlt.sources import DltResource
import pyarrow as pa
from typing import Optional


@dlt.source(name="chess")
def source(
    client: PyClient,
    players: List[str],
    start_month: Optional[str] = None,
    end_month: Optional[str] = None,
) -> Sequence[DltResource]:
    """
    A dlt source for the chess.com api. It groups several resources (in this case chess.com API endpoints) containing
    various types of data: user profiles or chess match results
    Args:
        players (List[str]): A list of the player usernames for which to get the data.
        start_month (str, optional): Filters out all the matches happening before `start_month`. Defaults to None.
        end_month (str, optional): Filters out all the matches happening after `end_month`. Defaults to None.
    Returns:
        Sequence[DltResource]: A sequence of resources that can be selected from including players_profiles,
        players_archives, players_games, players_online_status
    """
    return (
        players_profiles(client=client, players=players),
        # players_archives(p\layers),
        players_games(
            client=client, players=players, start_month=start_month, end_month=end_month
        ),
        # players_online_status(players),
    )


##  to get for benchmark: ("players_games", "players_profiles")


@dlt.resource(write_disposition="replace")
def players_profiles(client: PyClient, players: List[str]) -> pa.Table:
    res = get_player_profiles(client=client, players=players)
    yield res


@dlt.resource(write_disposition="append")
def players_games(
    client: PyClient,
    players: List[str],
    start_month: Optional[str] = None,
    end_month: Optional[str] = None,
) -> pa.Table:
    get_player_games(
        client=client, players=players, start_month=start_month, end_month=end_month
    )
    raise NotImplementedError("Rust function is in progress!")

    # yield games(players=players, start_month=start_month, end_month=end_month)


def load_players_games_example(start_month: str, end_month: str) -> None:
    """Constructs a pipeline that will load chess games of specific players for a range of months."""

    # configure the pipeline: provide the destination and dataset name to which the data should go
    pipeline = dlt.pipeline(
        pipeline_name="chess_pipeline_rs",
        destination="duckdb",
        dataset_name="chess_players_games_data",
    )
    # create the data source by providing a list of players and start/end month in YYYY/MM format
    client = PyClient()
    data = source(
        client=client,
        players=[
            "magnuscarlsen",
            "vincentkeymer",
            "dommarajugukesh",
            "rpragchess",
        ],
        start_month=start_month,
        end_month=end_month,
    )
    # load the "players_games" and "players_profiles" out of all the possible resources
    # info = pipeline.run(data.with_resources("players_games", "players_profiles"))
    info = pipeline.run(data.with_resources("players_games"))
    print(info)

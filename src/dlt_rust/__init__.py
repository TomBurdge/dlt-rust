from dlt_rust._core import hello_from_bin
import dlt
from typing import List, Sequence
from dlt.sources import DltResource
import pyarrow as pa
from typing import Optional


def main() -> None:
    print(hello_from_bin())


@dlt.source(name="chess")
def source(
    players: List[str], start_month: str = None, end_month: str = None
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
        players_profiles(players),
        # players_archives(p\layers),
        players_games(players, start_month=start_month, end_month=end_month),
        # players_online_status(players),
    )


##  to get for benchmark: ("players_games", "players_profiles")


@dlt.resource(write_disposition="replace")
def players_profiles(players: List[str]) -> pa.Table:
    """
    Yields player profiles for a list of player usernames.
    Args:
        players (List[str]): List of player usernames to retrieve profiles for.
    Yields:
        Iterator[TDataItem]: An iterator over player profiles data.
    """

    def usernames(players: List[str]) -> pa.Table:
        raise NotImplementedError("Rust function not yet written!")

    yield usernames(players=players)


@dlt.resource(write_disposition="append")
def players_games(
    players: List[str],
    start_month: Optional[str] = None,
    end_month: Optional[str] = None,
) -> pa.Table:
    def games(
        players: List[str],
        start_month: Optional[str] = None,
        end_month: Optional[str] = None,
    ) -> pa.Table:
        raise NotImplementedError("Rust function not yet written!")

    yield games(players=players, start_month=start_month, end_month=end_month)

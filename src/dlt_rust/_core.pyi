from typing import List
import pyarrow as pa
from typing import Optional

def get_player_profiles(players: List[str]) -> pa.Table: ...

class PyClient:
    def __init__(self) -> None: ...

def get_player_games(
    client: PyClient,
    players: List[str],
    start_month: Optional[str] = None,
    end_month: Optional[str] = None,
) -> pa.Table: ...

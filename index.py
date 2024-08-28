import requests
from typing import TypedDict


class UserInfo(TypedDict):
    bid: int
    name: str
    mbti: str
    gender: str


def get_user_info(bid: int) -> UserInfo | None:
    b = requests.get(f'https://code-api-pc.dao3.fun/user/profile/{bid}')
    d = requests.get(f'https://code-api-pc.dao3.fun/user/profile-info?userId=\
                     {bid}')
    basic = b.json()
    detail = d.json()

    return {
        1: 1
    }


get_user_info(12902610)

from dataclasses import dataclass
import json


def parse_data(value):
    if isinstance(value, list):
        parsed = []
        for member in value:
            parsed.append(parse_data(member))
        return parsed

    if isinstance(value, dict):
        for data_type, member in value.items():
            return member


def send_message(message: dict):
    serialized = json.dumps(message)
    print(serialized)


def receive_message() -> dict:
    serialized = input()
    data = json.loads(serialized)
    if "DataResponse" in data:
        return data["DataResponse"]

    raise ValueError("Unknown response received")


@dataclass
class Store:
    timestamps: list


@dataclass
class Material:
    pass


def request_data():
    message = {"DataRequest": None}
    send_message(message)

    data_response = receive_message()
    data_type, value = next(iter(data_response.items()))
    match data_type:
        case "Store":
            return Store(timestamps=parse_data(value["timestamps"]))
        case "Material":
            return Material()

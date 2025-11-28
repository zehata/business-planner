from pydantic import BaseModel


def request_data():
    print("request_data")
    return input()


def report(data):
    print("report")


class Material(BaseModel):
    name: str


material = Material(name="test")
serialized_material = material.model_dump_json()
print(serialized_material)
desered = Material.model_validate_json(serialized_material)
print(desered)

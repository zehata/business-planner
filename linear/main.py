from pprint import pprint
from business_planner_plugin import request_data


def main():
    data = request_data()
    pprint(data)


if __name__ == "__main__":
    main()

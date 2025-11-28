from business_planner_plugin import request_data, report


def main():
    data = request_data()
    if data == "some_data":
        print("processing")
    report("test")


if __name__ == "__main__":
    main()

## Start
Start the REPL with
```sh
> business-planner-cli
```
or start in interactive mode with
```sh
> business-planner-cli --interactive
```

## Adding and creating objects
### Store
Create a store with
```
> create store --name "Cookie jar"
Created store "Cookie jar" (8d055f2c-12f2-4197-8a85-c3d0001dc217)
```

### Production lines
Create a production line with
```
> create production-line --name "Cookies"
Created production line "Cookies" (3d1afc7f-9247-4606-9cda-b7c12fe810df)
```
---
Add a store to a production line with
```
> add store --name "Cookie jar" --line Cookies
```
or create a store directly with
```
> create store --name "Bread flour container" --line Cookies
```

## Listing and reading objects
List all production lines with
```
> list production-line
Cookies 3d1afc7f-9247-4606-9cda-b7c12fe810df
```
or use the shorthand
```
> list line
Cookies 3d1afc7f-9247-4606-9cda-b7c12fe810df
```
---
Read a production line with
```
> read line --id 3d1afc7f-9247-4606-9cda-b7c12fe810df
```
or more conveniently
```
> read line --name "Cookies"
```

## Deleting objects
Delete store with
```
> delete store --name "Bread flour container"
```
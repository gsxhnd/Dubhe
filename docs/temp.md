# mqtt

```text
`web ui` -> `api` -> `runtime`

runtime --|-- `Web UI`
runtime --|-- `HTTP controller api`
runtime --|-- `mqtt broker`
runtime --|-- `raft peer`

            > | ==> web ui
user client > |
            > | ==> controller api

mqtt broker <=> iot client

raft peer <=> raft peer
```

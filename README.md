# redeliver - retry failed Hasura events

This is a really simple CLI that redelivers failed [Hasura](hasura.io) event
triggers. All it does is leverage the provided API for [pg_get_event_logs][hasura_get_logs]
and [pg_redeliver_event][hasura_redeliver_event].

## Usage

```shell
Simple program to redeliver undelivered Hasura events

Usage: redeliver [OPTIONS] --event-trigger-name <EVENT_TRIGGER_NAME> <URL>

Arguments:
  <URL>  The hasura URL

Options:
  -a, --admin-secret <ADMIN_SECRET>
          Hasura's admin secret. If not present, use HASURA_GRAPHQL_ADMIN_SECRET env var
  -d, --data-file <DATA_FILE>
          The file in which the failed events are stored. If it is not present, will fetch the data
  -e, --event-trigger-name <EVENT_TRIGGER_NAME>
          The event trigger name
  -h, --help
          Print help
  -V, --version
          Print version
```

[hasura_get_logs]: https://hasura.io/docs/latest/api-reference/metadata-api/event-triggers/#metadata-pg-get-event-logs
[hasura_redeliver_event]: https://hasura.io/docs/latest/api-reference/metadata-api/event-triggers/#metadata-pg-redeliver-event

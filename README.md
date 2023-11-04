


## Grok
If you need to give the generated log files to ES for analysis, you can use this
```text
%{TIMESTAMP_ISO8601:timestamp} - 64 bytes from %{IPV4:ip_address}: seq=%{NUMBER:sequence} ttl=%{NUMBER:ttl} time=%{NUMBER:response_time} ms
```
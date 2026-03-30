# Time Based Key-Value Store

Design a time-based key-value store that supports set and get with timestamps, returning the most recent value at or before the queried timestamp.

## Example
set("foo","bar",1); get("foo",1) -> "bar"; get("foo",3) -> "bar"
set("foo","bar2",4); get("foo",4) -> "bar2"; get("foo",5) -> "bar2"

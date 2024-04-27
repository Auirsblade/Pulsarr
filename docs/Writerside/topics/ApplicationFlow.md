# Application Flow

<code-block lang="mermaid">

sequenceDiagram
    participant h as Homepage
    participant g as Group
    participant p as Profile
    participant m as Metrics
    participant s as Search
    h ->> g: "clicks on recent group rating"
    h ->> s: "search for album to rate"
    g ->> g: "search for album to rate within this group"

</code-block>
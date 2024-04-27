# Database Diagram

<code-block lang="mermaid">
erDiagram
    Group {
        int GroupID
        int RatingSystemID
        enum PublicStatus
    }
    User {
        int UserID
    }
    RatingSystem {
        int RatingSystemID
        enum MasterRatingType
        int AdditiveTypeMax
    }
    Group one to one RatingSystem: "has a"
    Group many to many User: "belongs to many"
    Permission {
        int PermissionID
        enum PermissionType
    }
    Rating {
        int RatingID
        int UserID
        int RatingSystemID
        int GroupID
        int MusicalEntityID
        int MasterRatingValue
    }
    RatingDetail {
        int RatingDetailID
        int RatingID
        int RatingSystemParameterID
        obj RatingDetailValue
    }
    RatingSystemParameter {
        int RatingSystemParameterID
        int RatingSystemID
        string nameOrDetailsOrRange
    }
    RatingSystem one to many RatingSystemParameter: "has many"
    RatingDetail many to many RatingSystemParameter: "is associated with"
    Rating one to many RatingDetail: "has many"
    Rating many to one Group: "belongs in"
    Rating many to one User: "rating made by"
    Rating one to one RatingSystem: "rated under"

</code-block>
-- Add migration script here
create table RatingSystem
(
    RatingSystemID serial
        constraint RatingSystem_pk
            primary key,
    Name text,
    SystemRatingType smallint,
    RatingMax numeric(8,3)
);

create table "user"
(
    UserID serial
        constraint User_pk
            primary key,
    Name text
);

create table "group"
(
    GroupID serial
        constraint Group_pk
            primary key,
    RatingSystemID integer not null
        constraint Group_RatingSystem_RatingSystemID_fk
            references RatingSystem (RatingSystemID),
    Name text,
    PrivacyType smallint
);

create table RatingSystemParameter
(
    RatingSystemParameterID serial
        constraint RatingSystemParameter_pk
            primary key,
    RatingSystemID integer not null
        constraint RatingSystemParameter_RatingSystem_RatingSystemID_fk
            references RatingSystem (RatingSystemID),
    Name text,
    ParameterRatingMax numeric(8,3)
);

create table Rating
(
    RatingID serial
        constraint Rating_pk
            primary key,
    UserID integer not null
        constraint Rating_User_UserID_fk
            references "user" (UserID),
    RatingSystemID integer not null
        constraint Rating_RatingSystem_RatingSystemID_fk
            references RatingSystem (RatingSystemID),
    GroupID integer not null
        constraint Rating_Group_GroupID_fk
            references "group" (GroupID),
    Comments text,
    Value numeric(8,3)
);

create table RatingDetail
(
    RatingDetailID serial
        constraint RatingDetail_pk
            primary key,
    RatingID integer not null
        constraint RatingDetail_Rating_RatingID_fk
            references Rating (RatingID),
    RatingSystemParameterID integer not null
        constraint RatingDetail_RatingSystemParameter_RatingSystemParameterID_fk
            references RatingSystemParameter (RatingSystemParameterID),
    Value numeric(8,3)
)
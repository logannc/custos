// create table channels (id integer primary key autoincrement not null, discord_id bigint not null, name text not null);
table! {
    channels {
        id -> Integer,
        // serenity::model::id::ChannelId
        // struct ChannelId(pub u64)
        discord_id -> BigInt,
        name -> Text,
    }
}

// create table scenes (id integer primary key autoincrement not null, channel_id integer not null, name text not null, order_idx integer not null, deleted_at timestamp);
table! {
    scenes {
        id -> Integer,
        channel_id -> Integer,
        name -> Text,
        order_idx -> Integer,
        deleted_at -> Timestamp,
    }
}

// create table beats (id integer primary key autoincrement not null, scene_id integer not null, kind text not null, blurb text);
table! {
    beats {
        id -> Integer,
        scene_id -> Integer,
        kind -> Text,
        blurb -> Nullable<Text>,
    }
}

// create table initiatives (id integer primary key autoincrement not null, channel_id integer not null, round integer not null default 0, active_position integer not null default 0, started_at timestamp not null, ended_at timestamp);
table! {
    initiatives {
        id -> Integer,
        channel_id -> Integer,
        round -> Integer,
        active_position -> Integer,
        started_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
    }
}


// create table initiative_members (id integer primary key autoincrement not null, initiative_id integer not null, name text not null, value integer not null, tiebreaker real not null default 0.0, deleted_at timestamp);
table! {
    initiative_members {
        id -> Integer,
        initiative_id -> Integer,
        name -> Text,
        value -> Integer,
        tiebreaker -> Float,
        deleted_at -> Nullable<Timestamp>,
    }
}
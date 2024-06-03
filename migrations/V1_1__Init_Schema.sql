CREATE SCHEMA data AUTHORIZATION postgres;

-- CREATE REP_KIND enum
CREATE TYPE rep_kind AS ENUM (
    'Normal',
    'Myo',
    'LengthenedPartial',
    'Drop'
);

-- CREATE REP TABLE
CREATE TABLE data.rep
(
    id BIGSERIAL PRIMARY KEY,
    kind rep_kind NOT NULL,
    "set" BIGINT NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

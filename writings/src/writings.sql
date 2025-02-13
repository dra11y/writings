-- Enums
CREATE TYPE writing_type AS ENUM ('book', 'hidden_word', 'prayer', 'tablet');
CREATE TYPE paragraph_style AS ENUM ('text', 'invocation', 'instruction');
CREATE TYPE author AS ENUM ('the_bab', 'bahaullah', 'abdul_baha');
CREATE TYPE hidden_word_kind AS ENUM ('arabic', 'persian');
CREATE TYPE prayer_kind AS ENUM ('obligatory', 'general', 'occasional', 'tablet');
CREATE TYPE prayer_source AS ENUM (
   'bahai_prayers',
   'additional_prayers_bahaullah',
   'additional_prayers_abdulbaha',
   'twenty_six_prayers_abdulbaha',
   'prayers_and_tablets_for_children'
);

-- Tables
CREATE TABLE writings (
    id SERIAL PRIMARY KEY,
    writing_type writing_type NOT NULL,
    ref_id TEXT NOT NULL,
    author author NOT NULL,
    title TEXT NOT NULL,
    subtitle TEXT,
    number INTEGER,
    paragraph INTEGER NOT NULL,
    style paragraph_style NOT NULL,
    text TEXT NOT NULL,
    hidden_word_kind hidden_word_kind,
    prayer_source prayer_source,
    prayer_kind prayer_kind,
    prelude TEXT, -- For Hidden Words
    invocation TEXT, -- For Hidden Words
    section TEXT[], -- For Prayers
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT valid_hidden_word CHECK (
        (writing_type = 'hidden_word' AND hidden_word_kind IS NOT NULL) OR
        (writing_type != 'hidden_word' AND hidden_word_kind IS NULL)
    ),
    CONSTRAINT valid_prayer CHECK (
        (writing_type = 'prayer' AND (prayer_kind IS NOT NULL AND prayer_source IS NOT NULL)) OR
        (writing_type != 'prayer' AND (prayer_kind IS NULL AND prayer_source IS NULL))
    ),
    CONSTRAINT section_not_empty CHECK (section IS NULL OR array_length(section, 1) > 0)
);

CREATE INDEX writings_writing_type_idx ON writings(writing_type);
CREATE INDEX writings_type_para_idx ON writings(writing_type, paragraph);

CREATE UNIQUE INDEX writings_ref_id_idx ON writings(ref_id);

CREATE INDEX writings_text_idx ON writings USING gin(to_tsvector('english', text));
CREATE INDEX writings_title_idx ON writings USING gin(to_tsvector('english', title));

-- unique paragraph number within title and number
CREATE UNIQUE INDEX writings_title_number_paragraph_idx ON writings (title, number, paragraph, );

-- Prayers: Unique based on prayer source and number
CREATE UNIQUE INDEX writings_prayers_idx ON writings 
   (prayer_source, number, section, paragraph) 
    WHERE writing_type = 'prayer' AND prayer_source = 'bahai_prayers';

CREATE UNIQUE INDEX writings_tablet_idx ON writings (title, paragraph)
    WHERE writing_type = 'tablet';

-- Fulltext search index
CREATE INDEX writings_fulltext_idx ON writings USING gin(
   to_tsvector('english', 
       title || ' ' || 
       COALESCE(subtitle, '') || ' ' || 
       COALESCE(prelude, '') || ' ' || 
       COALESCE(invocation, '') || ' ' || 
       text)
);

CREATE TRIGGER update_writings_timestamp
    BEFORE UPDATE ON writings
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();

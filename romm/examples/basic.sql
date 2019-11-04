CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS event (
    uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR NOT NULL,
    visitor_id INT NOT NULL,
    properties JSON NOT NULL,
    browser JSON NOT NULL
);

INSERT INTO event VALUES (
   'f186b680-237d-449d-ad66-ad91c4e53d3d',
  'pageview', '1',
  '{ "page": "/" }',
  '{ "name": "Chrome", "os": "Mac", "resolution": { "x": 1440, "y": 900 } }'
);

INSERT INTO event VALUES (
   uuid_generate_v4(),
  'pageview', '2',
  '{ "page": "/" }',
  '{ "name": "Firefox", "os": "Windows", "resolution": { "x": 1920, "y": 1200 } }'
);

INSERT INTO event VALUES (
   uuid_generate_v4(),
  'pageview', '1',
  '{ "page": "/account" }',
  '{ "name": "Chrome", "os": "Mac", "resolution": { "x": 1440, "y": 900 } }'
);

INSERT INTO event VALUES (
   uuid_generate_v4(),
  'purchase', '5',
  '{ "amount": 10 }',
  '{ "name": "Firefox", "os": "Windows", "resolution": { "x": 1024, "y": 768 } }'
);

INSERT INTO event VALUES (
   uuid_generate_v4(),
  'purchase', '15',
  '{ "amount": 200 }',
  '{ "name": "Firefox", "os": "Windows", "resolution": { "x": 1280, "y": 800 } }'
);

INSERT INTO event VALUES (
   uuid_generate_v4(),
  'purchase', '15',
  '{ "amount": 500 }',
  '{ "name": "Firefox", "os": "Windows", "resolution": { "x": 1280, "y": 800 } }'
);

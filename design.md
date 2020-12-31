# Design

This file is to outline how things should work (ex: http-routes)

## feed

### acquire feeds

- [ ] Read ompl
- [ ] Add feed(s) with routes

### fetching

- [ ] fetch all feeds in interval

## routes

API Path: `/api/` (not root, for maybe later web-ui)

### feeds

`/feed/many`:
- GET: Get all feeds that are on checked
- POST: Add Multiple feeds
- PUT?: Replace all feeds with payload
- DELETE: Delete all feeds

`/feed/import`:
- POST: Import from an ompl file

`/feed/export`:
- GET: Export all feeds into ompl format

`/feed/single`:
- GET: Get Feed information
- POST: Add an Feed
- PUT: Replace feed with payload (like modify options)
- DELETE: Delete an feed

`/feed/get`:
- GET: Get full feed since *payload*
- DELETE: Remove feed until *payload*

`/update`:
- GET: Get information about the update cycle
- POST: Update the update cycle information
- DELETE: Force Update now

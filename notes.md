# New architecture:

- Put all init logic into the buttons:
-- Initialize session, context and frame request in one async funtion.
-- call a session initialzed event on init.
-- Backup winit settings and override with ReactiveLowPower.
-- create system that disables xr cams when session not available and print a warning.
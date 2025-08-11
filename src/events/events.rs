use crate::create_events_plugin;
use crate::events::change_color::*;
use crate::events::change_counter::*;
use crate::events::change_location::*;
use crate::events::change_name::*;
use crate::events::grow_older::*;

create_events_plugin!(
    EventsPlugin,
    ChangeName,
    GrowOlder,
    ChangeLocation,
    OpenColorPicker,
    ChangeCounter,
);

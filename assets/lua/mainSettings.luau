local settings = world.get_resource(types.GameSettings)

settings.grid_cell_size = 100.0
settings.grid_width = 30
settings.grid_height = 30

settings.map_icon_base_scale = 0.3;

settings.camera_move_speed = 1000.0;
settings.camera_zoom_speed = 0.05;
settings.camera_min_zoom = 0.5;
settings.camera_max_zoom = 3.0;
settings.camera_edge_percent_x = 0.1;
settings.camera_edge_percent_y = 0.1;
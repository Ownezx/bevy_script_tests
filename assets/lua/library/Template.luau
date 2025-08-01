--!strict
local function spawnTemplate(template:Template, x:number, y:number, rotation:number?): number
    if not template then
        error("spawnTemplate: Attempted to spawn nil template")
    end

    local entity = world.spawn()

    local transform = construct(types.Transform, {})
    transform.translation.x = x;
    transform.translation.y = y;
    transform.scale.x = 1;
    transform.scale.y = 1;
    transform.scale.z = 1;
    if rotation and not template.fixed_rotation then
        transform.rotation.z = math.sin(rotation/2);
        transform.rotation.w = math.cos(rotation/2);
    end

    world.insert_component(entity, types.Transform, transform)

    if template.sensor then
        local instance = construct(types.SubsystemSensor, template.sensor)
        world.insert_component(entity, types.SubsystemSensor, instance)
    end
    
    if template.sensor_trace then
        local instance = construct(types.SensorTrace, template.sensor_trace)
        world.insert_component(entity, types.SensorTrace, instance)  
    end

    if template.map_icon then
        add_sprite_to_entity(entity, template.map_icon)
    end
    
    return entity
end

type SubsystemSensor = {
    range: number,
    noise_floor: number,
}

type SensorTrace = {
    biological: number,
    electrical: number,
    gravitational: number,
    size_x: number,
    size_y: number,
}

export type Template = {
    name:string,
    sensor_trace:SensorTrace?,
    sensor:SubsystemSensor?,
    map_icon:string?,
    fixed_rotation:boolean,
}

return {spawnTemplate = spawnTemplate}
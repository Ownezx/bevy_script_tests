local SubsystemSensor = types.SubsystemSensor

function on_script_loaded()
    local sensor = construct(SubsystemSensor, { range = 50.0, noise_floor = 0.1 })
    print(sensor)
end

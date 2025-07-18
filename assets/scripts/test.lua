function on_script_loaded()
    print("Hello world")
end

function on_update()
    print("test")
end

function on_click(x, y)
    print("Lua click position : {" .. x .. ", " .. y .. "}")
end

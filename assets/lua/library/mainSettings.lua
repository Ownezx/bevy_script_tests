local data = 2;

function on_script_reloaded(value)
    if value then
        data = value.data
        print("Kept previous data " .. data .. ", " .. value.additionalData[3])
    else
        print('I have not saved any state before uretsnloading')
    end
end

function on_script_unloaded()
    print("Goodbye world")
    local test = {}
    test.data = data;
    test.additionalData = { 3, 2, 1 };
    return test
end

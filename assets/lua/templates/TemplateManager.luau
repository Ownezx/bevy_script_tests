
local simple = require("./../templates/FirstTemplates")

function on_script_loaded()
    for _, val in pairs(simple) do
        add_template_to_database("simple", val)
    end
end

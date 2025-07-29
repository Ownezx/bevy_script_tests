local Template = require("./../library/Template")
local simple = require("./../templates/FirstTemplates")


function on_gm_action(name, template, x, y)
    Template.spawnTemplate(simple[template], x, y)
    print("Spawning entity " .. simple[template].name .. " at position: " .. x .. ", " .. y)
end

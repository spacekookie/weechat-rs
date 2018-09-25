#include "weechat-plugin.h"

WEECHAT_PLUGIN_NAME("myplug");
WEECHAT_PLUGIN_DESCRIPTION("It's a shameless plug");
WEECHAT_PLUGIN_AUTHOR("Katharina Fey <kookie@spacekookie.de>");
WEECHAT_PLUGIN_VERSION("6.66");
WEECHAT_PLUGIN_LICENSE("WTFPL");

struct t_weechat_plugin *weechat_plugin = NULL;

int plugging_cb(const void *pointer, void *data,
                   struct t_gui_buffer *buffer,
                   int argc, char **argv, char **argv_eol)
{
    weechat_printf(buffer, "Yo what's up y'all!!");
    return WEECHAT_RC_OK;
}

int weechat_plugin_init(struct t_weechat_plugin *plugin, 
                            int argc, char *argv[])
{
    weechat_plugin = plugin;
    weechat_hook_command("plug",
                          "Print some advertisement",
                          "object",
                          "object: Just some object you like\n",
                          NULL,
                          &plugging_cb, NULL, NULL);

    return WEECHAT_RC_OK;
}

int weechat_plugin_end(struct t_weechat_plugin *plugin)
{
    (void) plugin;
    return WEECHAT_RC_OK;
}
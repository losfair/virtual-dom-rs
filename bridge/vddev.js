window.vddev = {
    load: function(url) {
        return window.vdbridge.fetch_and_load_module(url);
    },
    show: function(context, handle) {
        let elem = document.getElementById("vddev-container");
        elem.innerHTML = "";
        elem.appendChild(context.bridge.get_resource(handle));
    }
};

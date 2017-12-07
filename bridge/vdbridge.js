(function() {

class VDBridge {
    constructor() {
        this.next_id = 0;
        this.id_pool = [];
        this.resources = [ null ];
    }

    add_resource(res) {
        let id;
        if(this.id_pool.length) {
            id = this.id_pool.shift();
        } else {
            id = this.next_id;
            this.next_id++;
            while(this.resources.length <= this.next_id) {
                this.resources.push(null);
            }
        }
        this.resources[id] = res;
        return id;
    }

    release_resource(id) {
        this.resources[id] = null;
        this.id_pool.push(id);
    }

    get_resource(id) {
        if(!this.resources[id]) {
            throw new TypeError("Invalid resource");
        }
        return this.resources[id];
    }

    create_element(tag) {
        let elem = document.createElement(tag);
        return this.add_resource(elem);
    }

    create_text_node(text) {
        let elem = document.createTextNode(text);
        return this.add_resource(elem);
    }

    append_child(parent, child) {
        let parent_elem = this.get_resource(parent);
        let child_elem = this.get_resource(child);
        parent_elem.appendChild(child_elem);
    }

    remove_child(parent, child) {
        let parent_elem = this.get_resource(parent);
        let child_elem = this.get_resource(child);
        parent_elem.removeChild(child_elem);
    }

    replace_child(parent, new_child, old_child) {
        let parent_elem = this.get_resource(parent);
        let new_child_elem = this.get_resource(new_child);
        let old_child_elem = this.get_resource(old_child);
        parent_elem.replaceChild(new_child_elem, old_child_elem);
    }

    set_property(id, key, value) {
        let elem = this.get_resource(id);
        elem[key] = value;
    }

    set_style(id, key, value) {
        let elem = this.get_resource(id);
        elem.style[key] = value;
    }

    release_node(id) {
        this.release_resource(id);
    }
}

function load_module_from_arraybuffer(buffer) {
    let context = {
        mem: null,
        bridge: new VDBridge(),
        instance: null
    };

    return WebAssembly.compile(buffer)
        .then(module => WebAssembly.instantiate(
            module,
            {
                env: build_env(context)
            }
        ))
        .then(inst => {
            context.mem = inst.exports.memory;
            context.instance = inst;
            return context;
        });
}

function fetch_and_load_module(url) {
    return fetch(url)
        .then(resp => resp.arrayBuffer())
        .then(data => load_module_from_arraybuffer(data));
}

function build_env(context) {
    let bridge = context.bridge;
    let read_string = (ptr) => {
        let buf = new Uint8Array(context.mem.buffer);
        let bytes = [];
        while(buf[ptr]) {
            bytes.push(buf[ptr]);
            ptr++;
        }
        let out = new Uint8Array(bytes);
        return new TextDecoder().decode(out);
    };

    return {
        vdbridge_create_element(tag) {
            return bridge.create_element(read_string(tag));
        },
        vdbridge_create_text_node(t) {
            return bridge.create_text_node(read_string(t));
        },
        vdbridge_append_child(parent, child) {
            return bridge.append_child(parent, child);
        },
        vdbridge_remove_child(parent, child) {
            return bridge.remove_child(parent, child);
        },
        vdbridge_replace_child(parent, new_child, old_child) {
            return bridge.replace_child(parent, new_child, old_child);
        },
        vdbridge_set_property(handle, k, v) {
            return bridge.set_property(handle, read_string(k), read_string(v));
        },
        vdbridge_set_style(handle, k, v) {
            return bridge.set_style(handle, read_string(k), read_string(v));
        },
        vdbridge_release_node(handle) {
            return bridge.release_node(handle);
        }
    };
}

window.vdbridge = {
    load_module_from_arraybuffer: load_module_from_arraybuffer,
    fetch_and_load_module: fetch_and_load_module
};

})();

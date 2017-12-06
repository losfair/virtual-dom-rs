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

function init() {
    let context = {
        mem: null,
        bridge: new VDBridge()
    };

    fetch("vdcore.wasm")
    .then(resp => resp.arrayBuffer())
    .then(data => WebAssembly.compile(data))
    .then(module => {
        return WebAssembly.instantiate(
            module,
            {
                env: build_env(context)
            }
        );
    })
    .then(inst => {
        console.log("init ok");
        context.mem = inst.exports.memory;
        let id = inst.exports.vdcore_hello_world();
        console.log(context.bridge.resources[id]);
    });
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

window.addEventListener("load", init);

})();

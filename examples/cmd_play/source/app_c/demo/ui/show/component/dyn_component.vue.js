_$pi.define("app_c/demo/ui/show/component/dyn_component.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./dyn_component.vue.tpl"], function (require, exports, module, direct_1, dyn_component_vue_tpl_1) {
    "use strict";

    exports.initMeta = exports.DynComponent2 = exports.DynComponent1 = void 0;
    class ComponentTag {
        constructor() {
            this.name = 'abcde';
            this.style = 'color:#ff0000;';
            this.className = 'testClass';
            this.index = 1;
            this.widget = DynComponent1;
        }
        change(e) {
            if (this.index !== e) {
                if (e === 0) {
                    this.widget = null;
                } else if (e === 1) {
                    this.widget = DynComponent1;
                } else if (e === 2) {
                    this.widget = DynComponent2;
                } else {
                    return;
                }
            }
            this.index = e;
        }
    }
    exports.default = ComponentTag;
    class DynComponent1 {
        firstPaint() {
            console.log('============== DynComponent1', this.name);
        }
    }
    exports.DynComponent1 = DynComponent1;
    class DynComponent2 {
        firstPaint() {
            console.log('============== DynComponent2', this.name);
        }
    }
    exports.DynComponent2 = DynComponent2;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/component/dyn_component.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/component/dyn_component.vue.wcss",
            _$cssHash = 3199076239;
        DynComponent1["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: dyn_component_vue_tpl_1.BW8, cssHash: _$cssHash };
        DynComponent2["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: dyn_component_vue_tpl_1.BW10, cssHash: _$cssHash };
        ComponentTag["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: dyn_component_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(ComponentTag, ["widget", "name", "className"]);
    direct_1.addField(DynComponent1, ['name']);
    direct_1.addField(DynComponent2, ['name']);
    direct_1.addField(ComponentTag, ['style']);
});
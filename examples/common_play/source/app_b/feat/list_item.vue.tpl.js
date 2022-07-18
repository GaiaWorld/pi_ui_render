_$pi.define("app_b/feat/list_item.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, btn_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 29 /*justifyContent*/, 2);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: w.info.name
                },
                style: {
                    width /*width*/: "auto",
                    height /*height*/: "80px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.handleClick($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.text = w.info.name;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n3, _$wAttrs);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});
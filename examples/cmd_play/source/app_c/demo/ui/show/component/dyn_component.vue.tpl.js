_$pi.define("app_c/demo/ui/show/component/dyn_component.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW10 = exports.BW8 = exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 30], [1, 0, 0, 1], [[0, 5], null, null, null, null], [0, 1, 0, 1], [0, 0, 1, 1], [0, 100], [2, 0]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createComponent(w, {
                attrs: {
                    is: w.widget,
                    name: w.name
                },
                class: direct_1.classHash(w.className)
            });
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setStyle(t.n4, 16 /*margin*/, staticObj[3]);
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 20);
            direct_1.setEvent(t.n4, "pointerclick", $event => {
                let r = w.change(1);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n4, "切换到动态组件1");
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n5, 16 /*margin*/, staticObj[3]);
            direct_1.setStyle(t.n5, 42 /*fontSize*/, 20);
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = w.change(2);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n5, "切换到动态组件2");
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[5]);
            direct_1.setStyle(t.n6, 16 /*margin*/, staticObj[3]);
            direct_1.setStyle(t.n6, 42 /*fontSize*/, 20);
            direct_1.setEvent(t.n6, "pointerclick", $event => {
                let r = w.change(0);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n6, "不显示组件");
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountComponent(t.n3, t.n2);
            direct_1.append(t.n4, t.n2);
            direct_1.append(t.n5, t.n2);
            direct_1.append(t.n6, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            var _$wAttrs = {};
            if (dirty0 & 3) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.is = w.widget;
                if (dirty0 & 2) _$attrs.name = w.name;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 4) _$wAttrs.class = direct_1.classHash(w.className);
            ;
            t.n3 = direct_1.patchAttrsForComponent(t.n3, _$wAttrs);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class BW8 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n8 = direct_1.createSpan();
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[6]);
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n8, 42 /*fontSize*/, 30);
            direct_1.extendAttr(t.n8, w, null, true);
            direct_1.setText(t.n8, "我是动态组件1");
            return this.n8;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n8, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let {} = w._$info;
            direct_1.extendAttr(t.n8, w);
            return this.n8;
        }
    }
    exports.BW8 = BW8;
    class BW10 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n10 = direct_1.createSpan();
            direct_1.setStyle(t.n10, 1 /*height*/, staticObj[6]);
            direct_1.setStyle(t.n10, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n10, 42 /*fontSize*/, 48);
            direct_1.extendAttr(t.n10, w, null, true);
            direct_1.setText(t.n10, "我是动态组件2");
            return this.n10;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n10, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let {} = w._$info;
            direct_1.extendAttr(t.n10, w);
            return this.n10;
        }
    }
    exports.BW10 = BW10;
});
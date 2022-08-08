_$pi.define("app_a/widget/menu/menu.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 2454166860, true);
            t.n3 = direct_1.createDiv();
            direct_1.setClass(t.n3, 349791717);
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: 'nav_bg' + (w.selected ? 2 : 1),
                    id: w.image
                },
                style: {
                    height /*height*/: "100%",
                    width /*width*/: "100%",
                    transform /*transform*/: w.selected ? 'scale(1.1, 1.2)' : 'scale(1, 1)'
                }
            });
            t.n5 = direct_1.createDiv();
            direct_1.setClass(t.n5, 3478786290);
            t.n6 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: 'nav_icon_' + w.image
                },
                style: {
                    width /*width*/: "100%",
                    height /*height*/: "100%",
                    top /*top*/: "8px",
                    transform /*transform*/: "scale(0.8,0.8)"
                }
            });
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 65 /*opacity*/, w.selected ? 1 : 0);
            direct_1.setClass(t.n7, 3770178283);
            t.n8 = direct_1.createSpan();
            direct_1.setClass(t.n8, 1065495870);
            direct_1.setText(t.n8, w.name);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n6);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n4, t.n3);
            direct_1.mountChildWidget(t.n6, t.n5);
            direct_1.append(t.n5, t.n3);
            direct_1.append(t.n8, t.n7);
            direct_1.append(t.n7, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 2454166860);
            var _$wAttrs = {};
            if (dirty0 & 3) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.name = 'nav_bg' + (w.selected ? 2 : 1);
                if (dirty0 & 2) _$attrs.id = w.image;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 1) {
                const _$style = {};
                if (dirty0 & 1) _$style.transform = w.selected ? 'scale(1.1, 1.2)' : 'scale(1, 1)';
                _$wAttrs.style = _$style;
            }
            ;
            direct_1.patchAttrsForWidget(t.n4, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 2) {
                const _$attrs = {};
                if (dirty0 & 2) _$attrs.name = 'nav_icon_' + w.image;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n6, _$wAttrs);
            if (dirty0 & 1) direct_1.setStyle(t.n7, 65 /*opacity*/, w.selected ? 1 : 0);
            if (dirty0 & 4) direct_1.setText(t.n8, w.name);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});
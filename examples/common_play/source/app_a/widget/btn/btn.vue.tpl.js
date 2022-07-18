_$pi.define("app_a/widget/btn/btn.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.type === 'cancel' ? B3 : B4;
        }

        s5() {
            let w = this.w;
            return w.metaImg ? B5 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setEvent(t.n2, "pointerclick", $event => {
                let r = w.clickBtn();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n2, "pointerdown", $event => {
                let r = w.down($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n2, "pointerup", $event => {
                let r = w.up($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n2, w, 1524638181, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            t.i5 = t.s5();
            t.n5 = direct_1.createIf(w, t.i5);
            t.n6 = direct_1.createDiv();
            direct_1.setClass(t.n6, 1932368835);
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 36 /*textShadow*/, w.shoadowColor + ' 2px 4px 0px');
            direct_1.setStyle(t.n7, 37 /*textStroke*/, w.strokeSize + 'px ' + w.strokeColor);
            direct_1.setStyle(t.n7, 42 /*fontSize*/, w.fontSize + 'px');
            direct_1.setStyle(t.n7, 30 /*color*/, w.fontColor);
            direct_1.setStyle(t.n7, 31 /*letterSpacing*/, w.spacing + 'px');
            direct_1.setStyle(t.n7, 44 /*fontWeight*/, w.weight);
            direct_1.setText(t.n7, w.text);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n5);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            t.n5.m(t.n2);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 1524638181);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            t.n5 = direct_1.patchIf(w, t.n5, t.i5, t.i5 = t.s5(), t.n2);
            if (dirty0 & 4) direct_1.setStyle(t.n7, 36 /*textShadow*/, w.shoadowColor + ' 2px 4px 0px');
            if (dirty0 & 24) direct_1.setStyle(t.n7, 37 /*textStroke*/, w.strokeSize + 'px ' + w.strokeColor);
            if (dirty0 & 32) direct_1.setStyle(t.n7, 42 /*fontSize*/, w.fontSize + 'px');
            if (dirty0 & 64) direct_1.setStyle(t.n7, 30 /*color*/, w.fontColor);
            if (dirty0 & 128) direct_1.setStyle(t.n7, 31 /*letterSpacing*/, w.spacing + 'px');
            if (dirty0 & 256) direct_1.setStyle(t.n7, 44 /*fontWeight*/, w.weight);
            if (dirty0 & 512) direct_1.setText(t.n7, w.text);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B5 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n5 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: w.metaImg
                },
                class: 3569701056
            });
            return this.n5;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n5);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n5, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            if (dirty0 & 2) {
                const _$attrs = {};
                if (dirty0 & 2) _$attrs.name = w.metaImg;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n5, _$wAttrs);
            return this.n5;
        }
    }
    class B4 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "btn_positive_bg"
                },
                class: 426648728
            });
            return this.n4;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n4, target, anchor);
        }
        p() {
            return this.n4;
        }
    }
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "btn_negative_bg"
                },
                class: 426648728
            });
            return this.n3;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, target, anchor);
        }
        p() {
            return this.n3;
        }
    }
});
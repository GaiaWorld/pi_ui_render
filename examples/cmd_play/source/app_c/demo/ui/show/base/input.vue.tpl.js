_$pi.define("app_c/demo/ui/show/base/input.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0.94, 0, 0, 1], [0, 200], [0, 40], [0.94, 0.56, 0.06, 1], [0, 30]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setAttr(t.n2, "on-tap", "test20");
            direct_1.setStyle(t.n2, 61 /*display*/, 0);
            direct_1.setStyle(t.n2, 6 /*position*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 65 /*opacity*/, 1);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createInput();
            direct_1.setAttr(t.n3, "id", "debug");
            direct_1.setAttr(t.n3, "type", "text");
            direct_1.setAttr(t.n3, "value", w.value1);
            direct_1.setStyle(t.n3, 42 /*fontSize*/, 18);
            direct_1.setStyle(t.n3, 30 /*color*/, staticObj[1]);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n3, 17 /*marginTop*/, staticObj[5]);
            direct_1.setEvent(t.n3, "change", $event => {
                let r = w.changeCall($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n3, "input", $event => {
                let r = w.inputChange($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n3, 3421068014);
            t.n4 = direct_1.createInput();
            direct_1.setAttr(t.n4, "id", "debug");
            direct_1.setAttr(t.n4, "type", "text");
            direct_1.setAttr(t.n4, "value", w.value2);
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 18);
            direct_1.setStyle(t.n4, 30 /*color*/, staticObj[1]);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n4, 17 /*marginTop*/, staticObj[5]);
            direct_1.setEvent(t.n4, "change", $event => {
                let r = w.changeCall2($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n4, "input", $event => {
                let r = w.inputChange($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n4, 3421068014);
            t.n5 = direct_1.createInput();
            direct_1.setAttr(t.n5, "id", "debug");
            direct_1.setAttr(t.n5, "type", "text");
            direct_1.setAttr(t.n5, "value", w.value3);
            direct_1.setStyle(t.n5, 42 /*fontSize*/, 18);
            direct_1.setStyle(t.n5, 30 /*color*/, staticObj[1]);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n5, 17 /*marginTop*/, staticObj[5]);
            direct_1.setEvent(t.n5, "change", $event => {
                let r = w.changeCall3($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n5, "input", $event => {
                let r = w.inputChange($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n5, 3421068014);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n4, t.n2);
            direct_1.append(t.n5, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            if (dirty0 & 1) direct_1.setAttr(t.n3, "value", w.value1);
            if (dirty0 & 2) direct_1.setAttr(t.n4, "value", w.value2);
            if (dirty0 & 4) direct_1.setAttr(t.n5, "value", w.value3);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});
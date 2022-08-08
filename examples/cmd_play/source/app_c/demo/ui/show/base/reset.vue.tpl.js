_$pi.define("app_c/demo/ui/show/base/reset.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 100], [0, 50], [1, 0, 0, 1], [0, 200]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createSpan();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n3, 42 /*fontSize*/, 20);
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.tap('width');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n3, "修改width");
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 20);
            direct_1.setEvent(t.n4, "pointerclick", $event => {
                let r = w.tap('left');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n4, "修改left");
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n5, 42 /*fontSize*/, 20);
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = w.tap('right');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n5, "修改right");
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n6, 7 /*top*/, staticObj[3]);
            direct_1.setStyle(t.n6, 6 /*position*/, 1);
            direct_1.setStyle(t.n6, 0 /*width*/, w.width);
            direct_1.setStyle(t.n6, 10 /*left*/, w.left);
            direct_1.setStyle(t.n6, 8 /*right*/, w.right);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
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
            if (dirty0 & 1) direct_1.setStyle(t.n6, 0 /*width*/, w.width);
            if (dirty0 & 2) direct_1.setStyle(t.n6, 10 /*left*/, w.left);
            if (dirty0 & 4) direct_1.setStyle(t.n6, 8 /*right*/, w.right);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});
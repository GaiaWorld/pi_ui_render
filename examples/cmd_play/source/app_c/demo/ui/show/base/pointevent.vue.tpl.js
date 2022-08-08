_$pi.define("app_c/demo/ui/show/base/pointevent.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 400], [0, 0, 1, 1], [0, 300], [1, 0, 0, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 61 /*display*/, 0);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 6 /*position*/, 1);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = alert("命中蓝色节点");
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 6 /*position*/, 1);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[3]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n4, 63 /*pointerEvents*/, 1);
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 6 /*position*/, 1);
            direct_1.setStyle(t.n5, 7 /*top*/, staticObj[1]);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n5, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n5, 42 /*fontSize*/, 20);
            direct_1.setText(t.n5, "点击红色节点，由于红色节点设置了\"pointer-events: none\",会穿透红色节点，命中其下的蓝色节点");
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
            let {} = w._$info;
            direct_1.extendAttr(t.n2, w);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});
_$pi.define("app_a/widget/tips/tips.tpl", ["require", "exports", "module", "../../../pi_gui/widget/vnode", "../../../pi_gui/widget/tpl"], function (require, exports, module, vnode_1, tpl_1) {
    "use strict";

    exports.tpl = void 0;
    const _path = 'app_a/widget/tips/tips.tpl';
    let staticNode;
    let staticDesc;
    let staticStyle;
    let staticFlag;
    exports.tpl = (_cfg, it, it1, w) => {
        if (!staticDesc) {
            staticDesc = {
                1: {
                    style: {
                        animation: [{
                            duration: 10000,
                            timingFunction: "linear",
                            delayTime: 0,
                            iteration: 1,
                            direction: "normal",
                            fillMode: "none",
                            name: "popAnim"
                        }]
                    },
                    clazz: ["e0"],
                    attrs: {
                        anim: "popAnim-end-animEnd"
                    },
                    tag: "div"
                },
                3: {
                    clazz: ["e1"],
                    tag: "div"
                },
                4: {
                    clazz: ["e2"],
                    tag: "div"
                }
            };
        }
        if (!staticNode) {
            staticNode = {};
        }
        let props = it;
        let cfg = _cfg;
        let state = it1;
        let widget = w;
        {
            let node_2;
            if (!!it.text) {
                var nodeDesc_5 = vnode_1.createDesc("span", ["e3"], null, undefined, null, null, "");
                tpl_1.addContent(it.text, nodeDesc_5);
                let node_5 = vnode_1.createVNode(nodeDesc_5, undefined, 1024);
                let node_4 = vnode_1.createVNode(staticDesc[4], node_5, 0);
                let node_3 = vnode_1.createVNode(staticDesc[3], node_4, 0);
                node_2 = vnode_1.createBlock(undefined, node_3, 67588, [node_5]);
            } else {
                node_2 = vnode_1.createBlock(null, null, 2052, []);
            }
            let node_1 = vnode_1.createVNode(staticDesc[1], node_2, 16);
            node_1.indexs = [node_2];
            return node_1;
        }
    };
});
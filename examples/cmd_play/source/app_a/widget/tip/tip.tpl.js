_$pi.define("app_a/widget/tip/tip.tpl", ["require", "exports", "module", "../../../pi_utils/util/hash", "../../../pi_gui/widget/vnode", "../../../pi_gui/compile/style", "../../../pi_gui/widget/tpl"], function (require, exports, module, hash_1, vnode_1, style_1, tpl_1) {
    "use strict";

    exports.tpl = void 0;
    const _path = 'app_a/widget/tip/tip.tpl';
    let staticNode;
    let staticDesc;
    let staticStyle;
    let staticFlag;
    exports.tpl = (_cfg, it, it1, w) => {
        if (!staticNode) {
            staticNode = {};
        }
        if (!staticStyle) {
            staticStyle = {
                "5": {
                    width: [1, 1],
                    height: [1, 1],
                    position: 1
                }
            };
        }
        let props = it;
        let cfg = _cfg;
        let state = it1;
        let widget = w;
        {
            var nodeDesc_1 = vnode_1.createDesc("div", ["e0"], null, {});
            var _$style = "";
            _$style += "width:";
            _$style += it.width || 60;
            _$style += "px;";
            _$style += "height:";
            _$style += it.height || 60;
            _$style += "px";
            nodeDesc_1.style = style_1.styleStr2Json(_$style, nodeDesc_1.style);
            let node_2;
            if (it && it1.redPoint) {
                let node_3;
                if (it.showW) {
                    var nodeDesc_4 = vnode_1.createDesc("widget", ["e1"], null, undefined, null, null, "");
                    var attrValue = "";
                    attrValue += it.showW;
                    nodeDesc_4.propsHash = hash_1.anyHash(it.showW, nodeDesc_4.propsHash);
                    nodeDesc_4.tag = attrValue;
                    tpl_1.addContent(it.show, nodeDesc_4);
                    nodeDesc_4.propsHash = hash_1.anyHash(it.show, nodeDesc_4.propsHash);
                    let node_4 = vnode_1.createVNode(nodeDesc_4, undefined, 1064);
                    node_3 = vnode_1.createBlock(undefined, node_4, 67588, [node_4]);
                } else {
                    var nodeDesc_5 = vnode_1.createDesc("ui-imgmap-base$$", null, null, staticStyle[5], null, null, "");
                    tpl_1.addContent(it.show, nodeDesc_5);
                    nodeDesc_5.propsHash = hash_1.anyHash(it.show, nodeDesc_5.propsHash);
                    let node_5 = vnode_1.createVNode(nodeDesc_5, undefined, 1032);
                    node_3 = vnode_1.createBlock(undefined, node_5, 133124, [node_5]);
                }
                node_2 = vnode_1.createBlock(undefined, node_3, 67588, [node_3]);
            } else {
                node_2 = vnode_1.createBlock(null, null, 2052, []);
            }
            let node_1 = vnode_1.createVNode(nodeDesc_1, node_2, 144);
            node_1.indexs = [node_1, node_2];
            return node_1;
        }
    };
});
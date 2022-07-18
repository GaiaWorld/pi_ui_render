_$pi.define("app_a/cfg/default_style", ["require", "exports", "module", "pi_gui/engine/style"], function (require, exports, module, style_1) {
    "use strict";

    exports.setDefaultFont = void 0;
    exports.setDefaultFont = vdocument => {
        vdocument.addFont("SOURCEHANSANSK-MEDIUM", 0, 0);
        vdocument.addFontFace("SOURCEHANSANSK-MEDIUM", "SOURCEHANSANSK-MEDIUM");
        style_1.Style.FontFamilyList.push("SOURCEHANSANSK-MEDIUM"); // 作用？
        // .0{font-size:36px;font-family:SOURCEHANSANSK-MEDIUM;color:#ffffff;flex-direction:row;}
        const css_bin = new Uint8Array([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 16, 66, 7, 0, 0, 0, 127, 215, 19, 135, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 24, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0]);
        vdocument.sheet.setDefaultStyle(css_bin);
    };
});
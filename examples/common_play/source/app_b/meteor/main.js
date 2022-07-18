_$pi.define("app_b/meteor/main", ["require", "exports", "module", "pi_common/ui/scratch"], function (require, exports, module, scratch) {
    "use strict";

    exports.pauseMeteor = exports.activeMeteor = exports.initMeteor = void 0;
    exports.initMeteor = () => {
        scratch.activate();
        scratch.useColor(1, 215 / 255, 49 / 255);
        // getGlobal().setPermanent(scratch.render);
    };
    exports.activeMeteor = () => scratch.activate();
    exports.pauseMeteor = () => scratch.frozen();
});
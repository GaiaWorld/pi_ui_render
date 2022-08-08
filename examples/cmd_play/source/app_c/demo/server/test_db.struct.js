_$pi.define("app_c/demo/server/test_db.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.TestDB4 = exports.TestDB3 = exports.Info = exports.TestDB2 = exports.TestDB = void 0;
    class TestDB extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return TestDB._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new TestDB();
            o.key = bb.readUtf8();
            o.value = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeUtf8(this.key);
            bb.writeUtf8(this.value);
            return bb;
        }
    }
    exports.TestDB = TestDB;
    TestDB._$info = new sinfo_1.StructInfo("app_c/demo/server/test_db.TestDB", 4097543039, new Map([["primary", "key"], ["db", "logfile"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("value", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(TestDB._$info.name_hash, TestDB, TestDB._$info.name);
    /**
    *测试数据库
    */
    class TestDB2 extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return TestDB2._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new TestDB2();
            o.key = bb.readArray(() => {
                return bb.readInt();
            });
            o.value = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeArray(this.key, el => {
                bb.writeInt(el);
            });
            bb.writeUtf8(this.value);
            return bb;
        }
    }
    exports.TestDB2 = TestDB2;
    TestDB2._$info = new sinfo_1.StructInfo("app_c/demo/server/test_db.TestDB2", 229029696, new Map([["primary", "key"], ["db", "logfile"], ["dbMonitors", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.Arr, new sinfo_1.EnumType(sinfo_1.Type.Usize)), null), new sinfo_1.FieldInfo("value", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(TestDB2._$info.name_hash, TestDB2, TestDB2._$info.name);
    class Info extends struct_mgr_1.Struct {
        static bonType() {
            return Info._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new Info();
            o.name = bb.readUtf8();
            o.addr = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeUtf8(this.name);
            bb.writeUtf8(this.addr);
            return bb;
        }
    }
    exports.Info = Info;
    Info._$info = new sinfo_1.StructInfo("app_c/demo/server/test_db.Info", 87684833, null, [new sinfo_1.FieldInfo("name", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("addr", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(Info._$info.name_hash, Info, Info._$info.name);
    /**
    *测试数据库
    */
    class TestDB3 extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            this.info && this.info.addMeta(mgr);
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
            this.info && this.info.removeMeta();
        }
        static bonType() {
            return TestDB3._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new TestDB3();
            o.uid = bb.readInt();
            o.info = o._$EnumTypeMap ? o._$EnumTypeMap(o.info) : Info.bonDecode(bb);
            o.url = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.uid);
            this.info.bonEncode(bb);
            bb.writeUtf8(this.url);
            return bb;
        }
    }
    exports.TestDB3 = TestDB3;
    TestDB3._$info = new sinfo_1.StructInfo("app_c/demo/server/test_db.TestDB3", 145508698, new Map([["primary", "uid"], ["db", "logfile"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("uid", new sinfo_1.EnumType(sinfo_1.Type.Usize), null), new sinfo_1.FieldInfo("info", new sinfo_1.EnumType(sinfo_1.Type.Struct, Info._$info), null), new sinfo_1.FieldInfo("url", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(TestDB3._$info.name_hash, TestDB3, TestDB3._$info.name);
    /**
    *测试数据库
    */
    class TestDB4 extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return TestDB4._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new TestDB4();
            o.uid = bb.readInt();
            o.info = bb.readMap(() => {
                return [bb.readUtf8(), bb.readUtf8()];
            });
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.uid);
            bb.writeMap(this.info, (k, v) => {
                bb.writeUtf8(k);
                bb.writeUtf8(v);
            });
            return bb;
        }
    }
    exports.TestDB4 = TestDB4;
    TestDB4._$info = new sinfo_1.StructInfo("app_c/demo/server/test_db.TestDB4", 1600984033, new Map([["primary", "uid"], ["db", "logfile"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("uid", new sinfo_1.EnumType(sinfo_1.Type.Usize), null), new sinfo_1.FieldInfo("info", new sinfo_1.EnumType(sinfo_1.Type.Map, [new sinfo_1.EnumType(sinfo_1.Type.Str), new sinfo_1.EnumType(sinfo_1.Type.Str)]), null)]);
    struct_mgr_1.structMgr.register(TestDB4._$info.name_hash, TestDB4, TestDB4._$info.name);
});
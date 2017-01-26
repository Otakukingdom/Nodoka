//
// Created by mistlight on 1/25/2017.
//

#include "Util.h"

static QString settingPath = "";

QString Core::getSettingPath() {
    if(settingPath == "") {
        settingPath = QStandardPaths::writableLocation(QStandardPaths::DataLocation);
    }

    return settingPath;
}

//
// Created by mistlight on 1/25/2017.
//

#include "Util.h"
#include <QDir>
#include <QCryptographicHash>
#include <QDebug>

static QString settingPath = "";

QString Core::getSettingPath() {
    if(settingPath == "") {
        settingPath = QStandardPaths::writableLocation(QStandardPaths::DataLocation);
    }

    return settingPath;
}

QString Core::getUniqueSettingPath(QString stringToHash) {
    QCryptographicHash hash(QCryptographicHash::Sha1);
    hash.addData(stringToHash.toLocal8Bit());
    QByteArray byteResult = hash.result();
    auto strResult = QString(byteResult.toHex());

    QStringRef firstStrRef(&strResult, 0, 2);
    auto first = firstStrRef.toString();

    QStringRef secondStrRef(&strResult, 2, -1);
    auto rest = secondStrRef.toString();

    auto pathToCreate = QDir(getSettingPath() + "/" + first).absolutePath();
    createPathIfNotExists(pathToCreate);

    auto resultPath = QDir(pathToCreate + "/" + rest).absolutePath();
    return resultPath;
}

QString Core::convertTimeToString(long long timeToAdd) {
    // update the label
    QTime time(0, 0);
    time = time.addMSecs(static_cast<int>(timeToAdd));
    QString timeInFormat = time.toString("hh:mm:ss");
    return timeInFormat;
}

void Core::createPathIfNotExists(QString path) {
    QDir dir(path);

    if(!dir.exists()) {
        dir.mkpath(".");
    }
}

void Core::createSettingPathIfNotExists() {
    createPathIfNotExists(getSettingPath());
}

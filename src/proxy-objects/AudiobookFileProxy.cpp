//
// Created by mistlight on 1/7/17.
//

#include "AudiobookFileProxy.h"
#include <QSqlError>
#include <QDebug>
#include <src/core/Util.h>
#include <QFile>
#include <QtCore/QCryptographicHash>

AudiobookFileProxy::AudiobookFileProxy(QSqlRecord record, Core::Setting* setting) {
    this->record = record;
    this->setting = setting;
    this->isNull = false;

    auto path = this->path();
    // if we have an empty path, it's not a valid AudiobookFile record...
    if(path == "") {
        this->isNull;
        return;
    }

    auto pathToSettings = Core::getUniqueSettingPath(path);

    this->currentFileSetting = QSharedPointer<QSettings>(new QSettings(pathToSettings, QSettings::IniFormat));
}

AudiobookFileProxy::AudiobookFileProxy() {
    this->record = QSqlRecord();
    this->isNull = true;
}

QString AudiobookFileProxy::path() {
    return this->record.value("full_path").toString();
}

bool AudiobookFileProxy::getNullState() {
    return this->isNull;
}

QString AudiobookFileProxy::name() {
    return this->record.value("name").toString();
}

void AudiobookFileProxy::setProperty(MediaProperty property) {
    this->mediaProperty = property;

    // once media property is set, check if there is elements where we
    // can start writing to the file storage
    if(this->mediaProperty.isNullObject()) {
        // if the media property is null, we don't have to do anything
        return;
    }

    auto currentDuration = this->getMediaDuration();
    if(currentDuration > 0) {
        this->setMediaDuration(currentDuration);
    }
}

void AudiobookFileProxy::setMediaDuration(const long long duration) {
    this->currentFileSetting->setValue("duration", duration);
    this->currentFileSetting->sync();
}

long long AudiobookFileProxy::getMediaDuration() {
    auto durationFromFile = this->currentFileSetting->value("duration").toLongLong();

    if(durationFromFile > 0) {
        return durationFromFile;
    }


    // if we have a null object, then we shouldn't return a valid
    // duration
    if(this->mediaProperty.isNullObject()) {
        return -1;
    }

    return this->mediaProperty.getDuration();
}

bool AudiobookFileProxy::isPropertyParsed() {
    if(this->mediaProperty.isNullObject()) {
        return false;
    } else {
        return true;
    }
}

void AudiobookFileProxy::setAsCurrent() {
    if(!this->mediaProperty.isNullObject()) {
        int audiobookId = this->record.value("audiobook_id").toInt();
        auto path = this->record.value("full_path").toString();
        QString queryString = "UPDATE audiobooks SET selected_file=? WHERE id=?";
        QSqlQuery query;
        query.prepare(queryString);
        query.addBindValue(path);
        query.addBindValue(audiobookId);
        if(!query.exec()) {
            qWarning() << "audiobook save query failed: "
                       << query.lastError().driverText()
                       << ", " << query.lastError().databaseText();
        }

        // set the current audiobook in the ini file
        this->setting->setCurrentAudiobook(audiobookId);
    }
}

void AudiobookFileProxy::saveCurrentTime(long long currentTime) {
    auto path = this->record.value("full_path").toString();

    bool completeness = false;
    auto duration = this->getMediaDuration();
    double calcCompleteness = 0;
    if(duration > 0) {
        auto oldCompleteness = this->getCompleteness();
        calcCompleteness = (double)currentTime/(double)duration * 100.0;

        if(calcCompleteness > oldCompleteness) {
            completeness = true;
        }
    }

    this->currentFileSetting->setValue("currentTime", currentTime);
    if(completeness) {
        this->currentFileSetting->setValue("completeness", calcCompleteness);

        if(this->completenssFunctionSet) {
            this->totalCompletenessUpdateFunction();
        }
    }
}

long long AudiobookFileProxy::getCurrentTime() {
    return this->currentFileSetting->value("currentTime").toLongLong();
}

bool AudiobookFileProxy::currentTimeNull() {
    return this->currentFileSetting->value("currentTime").isNull();
}

bool AudiobookFileProxy::hasNextFile() {
    int currentPosition = this->record.value("position").toInt();
    int audiobookId = this->record.value("audiobook_id").toInt();

    int nextPosition = currentPosition + 1;

    QString queryString = "SELECT * FROM audiobook_file WHERE position=? AND audiobook_id=?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(nextPosition);
    query.addBindValue(audiobookId);

    if(!query.exec()) {
        qWarning() << "audiobook retrieve next file state failed: "
                   << query.lastError().driverText()
                   << ", " << query.lastError().databaseText();
        return false;
    }

    if(query.next()) {
        return true;
    } else {
        return false;
    }
}

AudiobookFileProxy AudiobookFileProxy::getNextFile() {
    if(!hasNextFile()) {
        return AudiobookFileProxy();
    }

    int currentPosition = this->record.value("position").toInt();
    int audiobookId = this->record.value("audiobook_id").toInt();

    int nextPosition = currentPosition + 1;

    QString queryString = "SELECT * FROM audiobook_file WHERE position=? AND audiobook_id=?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(nextPosition);
    query.addBindValue(audiobookId);

    if(!query.exec()) {
        qWarning() << "audiobook retrieve next file failed: "
                   << query.lastError().driverText()
                   << ", " << query.lastError().databaseText();
        return AudiobookFileProxy();
    }

    if(query.next()) {
        return AudiobookFileProxy(query.record(), this->setting);
    } else {
        qWarning() << "audiobook retrieve next file failed: (next file record is empty)";
        return AudiobookFileProxy();
    }
}

QSqlRecord AudiobookFileProxy::getRecord() {
    return this->record;
}

int AudiobookFileProxy::getCompleteness() {
    return (int)round(this->currentFileSetting->value("completeness").toDouble());
}

void AudiobookFileProxy::setAsComplete() {
    this->currentFileSetting->setValue("completeness", 100);
}

void AudiobookFileProxy::setTotalDurationUpdateFunction(std::function<void()> audiobookProxyUpdateFunction) {
    this->durationFunctionSet = true;
    this->totalDurationUpdateFunction = audiobookProxyUpdateFunction;
}


QString AudiobookFileProxy::calcCheckSum() {
    QByteArray byteArray;
    QFile f(this->path());
    if (f.open(QFile::ReadOnly)) {
        QCryptographicHash hash(QCryptographicHash::Sha1);
        if (hash.addData(&f)) {
            byteArray = hash.result();
        }
    }

    if(!byteArray.isNull()) {
        auto sha1Hash = byteArray.toHex();

        return QString::fromLocal8Bit(sha1Hash);
    } else {
        return QString();
    }
}

void AudiobookFileProxy::calcAndWriteCheckSum(bool forced) {
    if(forced || !this->currentFileSetting->contains("checkSum")) {
        auto checkSum = calcCheckSum();
        this->currentFileSetting->setValue("checkSum", checkSum);
        this->currentFileSetting->sync();
    }
}

void AudiobookFileProxy::setCompletenessUpdateFunction(std::function<void()> func) {
    this->completenssFunctionSet = true;
    this->totalCompletenessUpdateFunction = func;
}

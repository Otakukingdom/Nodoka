//
// Created by mistlight on 1/7/17.
//

#include "AudiobookFileProxy.h"
#include <QSqlError>
#include <QDebug>

AudiobookFileProxy::AudiobookFileProxy(QSqlRecord record, Core::Setting* setting) {
    this->record = record;
    this->setting = setting;
    this->isNull = false;
}

QString AudiobookFileProxy::path() {
    return this->record.value("full_path").toString();
}

bool AudiobookFileProxy::getNullState() {
    return this->isNull;
}

AudiobookFileProxy::AudiobookFileProxy() {
    this->record = QSqlRecord();
    this->isNull = true;
}

QString AudiobookFileProxy::name() {
    return this->record.value("name").toString();
}

void AudiobookFileProxy::setProperty(MediaProperty property) {
    this->mediaProperty = property;
}

long long AudiobookFileProxy::getMediaDuration() {
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
        setting->setCurrentAudiobook(audiobookId);

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
    }
}

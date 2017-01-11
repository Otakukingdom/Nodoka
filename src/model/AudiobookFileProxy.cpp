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

void AudiobookFileProxy::saveCurrentTime(long long currentTime) {
    auto path = this->record.value("full_path").toString();

    QString queryString = "UPDATE audiobook_file SET seek_position=? WHERE full_path=?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(currentTime);
    query.addBindValue(path);

    if(!query.exec()) {
        qWarning() << "audiobook save currentTime failed: "
                   << query.lastError().driverText()
                   << ", " << query.lastError().databaseText();
    }
}

long long AudiobookFileProxy::getCurrentTime() {
    auto path = this->record.value("full_path").toString();
    QString queryString = "SELECT full_path, seek_position FROM audiobook_file WHERE full_path=?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(path);

    if(!query.exec()) {
        qWarning() << "audiobook retrieve currentTime failed: "
                   << query.lastError().driverText()
                   << ", " << query.lastError().databaseText();
        return -1;
    }

    if(query.next())  {
        auto result = query.record();
        return result.value("seek_position").toInt();
    } else {
        qWarning() << "audiobook retrieve currentTime failed: (no result)";
        return -1;
    }
}

bool AudiobookFileProxy::currentTimeNull() {
    auto path = this->record.value("full_path").toString();
    QString queryString = "SELECT full_path, seek_position FROM audiobook_file WHERE full_path=?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(path);

    if(!query.exec()) {
        qWarning() << "audiobook retrieve currentTime null state failed: "
                   << query.lastError().driverText()
                   << ", " << query.lastError().databaseText();
        return true;
    }

    if(query.next())  {
        auto result = query.record();
        return result.value("seek_position").isNull();
    } else {
        qWarning() << "audiobook retrieve currentTime null state failed: (no result)";
        return true;
    }
}

bool AudiobookFileProxy::hasNextFile() {
    qDebug() << "hasNextFile called()";
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
    qDebug() << "hasNextFile executed";

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

//
// Created by mistlight on 1/7/17.
//

#ifndef NODOKANATIVE_AUDIOBOOKRECORDPROXY_H
#define NODOKANATIVE_AUDIOBOOKRECORDPROXY_H


#include <QSqlRecord>
#include <QSqlQuery>
#include <QVariant>
#include <src/core/Setting.h>
#include "MediaProperty.h"

class AudiobookFileProxy {
    Core::Setting* setting;
    QSqlRecord record;
    bool isNull;
    MediaProperty mediaProperty;

public:
    AudiobookFileProxy(QSqlRecord record, Core::Setting* setting);
    AudiobookFileProxy();
    QString path();
    QString name();
    bool getNullState();
    bool isPropertyParsed();
    void setAsCurrent();
    void saveCurrentTime(long long currentTime);
    void setAsComplete();
    long long getCurrentTime();
    bool currentTimeNull();
    int getCompleteness();

    bool hasNextFile();
    AudiobookFileProxy getNextFile();
    QSqlRecord getRecord();

    void setProperty(MediaProperty property);
    long long getMediaDuration();

};

Q_DECLARE_METATYPE(AudiobookFileProxy);

#endif //NODOKANATIVE_AUDIOBOOKRECORDPROXY_H

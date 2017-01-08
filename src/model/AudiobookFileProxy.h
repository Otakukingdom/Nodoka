//
// Created by mistlight on 1/7/17.
//

#ifndef NODOKANATIVE_AUDIOBOOKRECORDPROXY_H
#define NODOKANATIVE_AUDIOBOOKRECORDPROXY_H


#include <QSqlRecord>
#include <QVariant>
#include "MediaProperty.h"

class AudiobookFileProxy {
    QSqlRecord record;
    bool isNull;
    MediaProperty mediaProperty;

public:
    AudiobookFileProxy(QSqlRecord record);
    AudiobookFileProxy();
    QString path();
    QString name();
    bool getNullState();
    bool isPropertyParsed();

    void setProperty(MediaProperty property);
    double getMediaDuration();

};

Q_DECLARE_METATYPE(AudiobookFileProxy);

#endif //NODOKANATIVE_AUDIOBOOKRECORDPROXY_H

//
// Created by mistlight on 1/7/17.
//

#ifndef NODOKANATIVE_AUDIOBOOKRECORDPROXY_H
#define NODOKANATIVE_AUDIOBOOKRECORDPROXY_H


#include <QSqlRecord>
#include <QVariant>

class AudiobookFileProxy {
    QSqlRecord record;
    bool isNull;

public:
    AudiobookFileProxy(QSqlRecord record);
    AudiobookFileProxy();
    QString path();
    QString name();
    bool getNullState();

};

Q_DECLARE_METATYPE(AudiobookFileProxy);

#endif //NODOKANATIVE_AUDIOBOOKRECORDPROXY_H

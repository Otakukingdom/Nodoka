//
// Created by mistlight on 3/9/2017.
//

#ifndef NODOKA_DATABASEMODEL_H
#define NODOKA_DATABASEMODEL_H

#include <memory>
#include <QJsonObject>
#include <src/core/DatabaseConnect.h>
#include <src/core/Util.h>
#include <QtCore/QJsonArray>
#include <QtCore/QJsonObject>

namespace Database {
    class DatabaseModel {
    protected:
        std::shared_ptr<Core::DatabaseInstance> dbInstance;

    public:
        void writeObject(QString key, QJsonObject value);
        void writeArray(QString key, QJsonArray value);
        void printData();
        void printValue(QString key);

        DatabaseModel(std::shared_ptr<Core::DatabaseInstance> dbInstance);


        virtual void save() = 0;
        virtual void load() = 0;
    };
}


#endif //NODOKA_DATABASEMODEL_H

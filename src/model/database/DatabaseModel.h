//
// Created by mistlight on 3/9/2017.
//

#ifndef NODOKA_DATABASEMODEL_H
#define NODOKA_DATABASEMODEL_H


class DatabaseModel {


public:
    virtual void save() = 0;
    virtual void load() = 0;
};


#endif //NODOKA_DATABASEMODEL_H

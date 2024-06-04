import Joi from 'joi';

export const componentCreateSchema = Joi.object({
    name : Joi.string().required().messages({
        'string.name': 'Name is required.',
    }),
    identifier : Joi.string().required().messages({
        'string.empty': 'Identifier is required.',
    }),
    fields : Joi.array()
});
